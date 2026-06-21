#!/usr/bin/env ts-node
/**
 * IDL Compatibility Checker
 * Run in CI before any PR merge
 * Detects breaking changes between old and new IDL versions
 */

import * as fs from 'fs';
import * as path from 'path';
import { createHash } from 'crypto';

interface IdlField {
    name: string;
    type: string | object;
}

interface IdlAccount {
    name: string;
    type: {
        kind: string;
        fields: IdlField[];
    };
}

interface IdlInstruction {
    name: string;
    accounts: Array<{ name: string; isMut: boolean; isSigner: boolean }>;
    args: Array<{ name: string; type: string | object }>;
}

interface Idl {
    version: string;
    name: string;
    accounts: IdlAccount[];
    instructions: IdlInstruction[];
    errors?: Array<{ code: number; name: string; msg: string }>;
}

function loadIdl(filePath: string): Idl {
    if (!fs.existsSync(filePath)) {
        throw new Error(`IDL file not found: ${filePath}`);
    }
    return JSON.parse(fs.readFileSync(filePath, 'utf-8'));
}

function typeToString(type: string | object): string {
    return typeof type === 'string' ? type : JSON.stringify(type);
}

function checkBreakingChanges(oldIdl: Idl, newIdl: Idl): string[] {
    const errors: string[] = [];
    const warnings: string[] = [];

    console.log(`Comparing IDL versions: ${oldIdl.version} → ${newIdl.version}`);
    console.log('');

    // Check account changes
    for (const oldAccount of oldIdl.accounts) {
        const newAccount = newIdl.accounts.find(a => a.name === oldAccount.name);

        if (!newAccount) {
            errors.push(`BREAKING: Account '${oldAccount.name}' was REMOVED`);
            continue;
        }

        const oldFields = oldAccount.type.fields;
        const newFields = newAccount.type.fields;

        // Check for field removals
        for (const oldField of oldFields) {
            const newField = newFields.find(f => f.name === oldField.name);
            if (!newField) {
                errors.push(`BREAKING: Field '${oldField.name}' REMOVED from '${oldAccount.name}'`);
                continue;
            }

            const oldType = typeToString(oldField.type);
            const newType = typeToString(newField.type);

            if (oldType !== newType) {
                errors.push(`BREAKING: Field '${oldField.name}' type changed from '${oldType}' to '${newType}' in '${oldAccount.name}'`);
            }
        }

        // Check for field reordering
        for (let i = 0; i < Math.min(oldFields.length, newFields.length); i++) {
            if (oldFields[i].name !== newFields[i].name) {
                errors.push(`BREAKING: Fields REORDERED in '${oldAccount.name}'. Expected '${oldFields[i].name}' at position ${i}, found '${newFields[i].name}'`);
                break;
            }
        }

        // Check for new fields (safe if appended at end)
        if (newFields.length > oldFields.length) {
            const newFieldNames = newFields.slice(oldFields.length).map(f => f.name);
            warnings.push(`SAFE: New fields appended to '${oldAccount.name}': ${newFieldNames.join(', ')}`);
        }
    }

    // Check instruction compatibility
    for (const oldIx of oldIdl.instructions) {
        const newIx = newIdl.instructions.find(i => i.name === oldIx.name);
        if (!newIx) {
            errors.push(`BREAKING: Instruction '${oldIx.name}' was REMOVED`);
            continue;
        }

        // Check account requirements changed
        for (const oldAcc of oldIx.accounts) {
            const newAcc = newIx.accounts.find(a => a.name === oldAcc.name);
            if (!newAcc) {
                errors.push(`BREAKING: Account '${oldAcc.name}' REMOVED from instruction '${oldIx.name}'`);
                continue;
            }
            if (oldAcc.isMut !== newAcc.isMut || oldAcc.isSigner !== newAcc.isSigner) {
                errors.push(`BREAKING: Account '${oldAcc.name}' constraints changed in '${oldIx.name}'`);
            }
        }
    }

    // Check for new instructions (safe)
    const newInstructions = newIdl.instructions.filter(
        ni => !oldIdl.instructions.find(oi => oi.name === ni.name)
    );
    if (newInstructions.length > 0) {
        warnings.push(`SAFE: New instructions added: ${newInstructions.map(i => i.name).join(', ')}`);
    }

    return [...errors, ...warnings];
}

function computeLayoutHash(fields: IdlField[]): string {
    const layout = fields.map(f => `${f.name}:${typeToString(f.type)}`).join('|');
    return createHash('sha256').update(layout).digest('hex').substring(0, 16);
}

function checkLayoutDrift(oldIdl: Idl, newIdl: Idl): string[] {
    const drifts: string[] = [];

    for (const oldAccount of oldIdl.accounts) {
        const newAccount = newIdl.accounts.find(a => a.name === oldAccount.name);
        if (!newAccount) continue;

        const oldHash = computeLayoutHash(oldAccount.type.fields);
        const newHash = computeLayoutHash(newAccount.type.fields);

        if (oldHash !== newHash) {
            const oldSize = oldAccount.type.fields.length;
            const newSize = newAccount.type.fields.length;

            if (newSize > oldSize) {
                drifts.push(`DRIFT: '${oldAccount.name}' layout changed (fields added: ${oldSize} → ${newSize})`);
            } else if (newSize < oldSize) {
                drifts.push(`CRITICAL: '${oldAccount.name}' layout changed (fields REMOVED: ${oldSize} → ${newSize})`);
            } else {
                drifts.push(`CRITICAL: '${oldAccount.name}' layout changed (fields reordered/modified)`);
            }
        }
    }

    return drifts;
}

function main() {
    const args = process.argv.slice(2);
    const oldIdlPath = args[0] || 'target/idl/old.json';
    const newIdlPath = args[1] || 'target/idl/new.json';

    console.log('🔍 IDL Compatibility Check');
    console.log('═══════════════════════════');
    console.log('');

    let oldIdl: Idl;
    let newIdl: Idl;

    try {
        oldIdl = loadIdl(oldIdlPath);
        newIdl = loadIdl(newIdlPath);
    } catch (e) {
        console.error(`❌ Error loading IDL: ${e}`);
        process.exit(1);
    }

    const changes = checkBreakingChanges(oldIdl, newIdl);
    const drifts = checkLayoutDrift(oldIdl, newIdl);

    let hasErrors = false;

    for (const change of changes) {
        if (change.startsWith('BREAKING') || change.startsWith('CRITICAL')) {
            console.error(`❌ ${change}`);
            hasErrors = true;
        } else {
            console.log(`✅ ${change}`);
        }
    }

    for (const drift of drifts) {
        if (drift.startsWith('CRITICAL')) {
            console.error(`❌ ${drift}`);
            hasErrors = true;
        } else {
            console.log(`⚠️  ${drift}`);
        }
    }

    console.log('');

    if (hasErrors) {
        console.error('❌ IDL compatibility check FAILED');
        console.error('Fix breaking changes before merging.');
        process.exit(1);
    }

    console.log('✅ IDL compatibility check PASSED');
    console.log('No breaking changes detected. Safe to merge.');
    process.exit(0);
}

main();
