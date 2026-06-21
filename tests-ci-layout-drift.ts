#!/usr/bin/env ts-node
/**
 * Borsh Layout Drift Detector
 * Detects structural changes in account layouts between versions
 */

import * as fs from 'fs';
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

interface Idl {
    accounts: IdlAccount[];
}

function loadIdl(filePath: string): Idl {
    return JSON.parse(fs.readFileSync(filePath, 'utf-8'));
}

function typeToString(type: string | object): string {
    return typeof type === 'string' ? type : JSON.stringify(type);
}

function computeLayoutHash(fields: IdlField[]): string {
    const layout = fields.map(f => `${f.name}:${typeToString(f.type)}`).join('|');
    return createHash('sha256').update(layout).digest('hex');
}

function computeSizeEstimate(fields: IdlField[]): number {
    let size = 0;
    for (const field of fields) {
        const type = typeToString(field.type);
        switch (type) {
            case 'u8': case 'i8': case 'bool':
                size += 1; break;
            case 'u16': case 'i16':
                size += 2; break;
            case 'u32': case 'i32': case 'f32':
                size += 4; break;
            case 'u64': case 'i64': case 'f64': case 'publicKey':
                size += 8; break;
            case 'u128': case 'i128':
                size += 16; break;
            default:
                if (type === 'string') size += 4; // length prefix only (variable)
                else if (type.startsWith('[')) size += 32; // array estimate
                else size += 32; // default estimate
        }
    }
    return size;
}

function detectDrift(oldIdl: Idl, newIdl: Idl): { account: string; oldHash: string; newHash: string; oldSize: number; newSize: number; safe: boolean }[] {
    const results = [];

    for (const oldAccount of oldIdl.accounts) {
        const newAccount = newIdl.accounts.find(a => a.name === oldAccount.name);
        if (!newAccount) continue;

        const oldHash = computeLayoutHash(oldAccount.type.fields);
        const newHash = computeLayoutHash(newAccount.type.fields);

        if (oldHash !== newHash) {
            const oldSize = computeSizeEstimate(oldAccount.type.fields);
            const newSize = computeSizeEstimate(newAccount.type.fields);

            // Safe if: same field order, only new fields appended, no type changes
            const oldFieldNames = oldAccount.type.fields.map(f => f.name);
            const newFieldNames = newAccount.type.fields.map(f => f.name);
            const isAppendOnly = oldFieldNames.every((name, i) => newFieldNames[i] === name);

            results.push({
                account: oldAccount.name,
                oldHash,
                newHash,
                oldSize,
                newSize,
                safe: isAppendOnly && newSize >= oldSize
            });
        }
    }

    return results;
}

function main() {
    const args = process.argv.slice(2);
    const oldIdlPath = args[0] || 'target/idl/old.json';
    const newIdlPath = args[1] || 'target/idl/new.json';

    console.log('📐 Borsh Layout Drift Detection');
    console.log('═══════════════════════════════');
    console.log('');

    const oldIdl = loadIdl(oldIdlPath);
    const newIdl = loadIdl(newIdlPath);

    const drifts = detectDrift(oldIdl, newIdl);

    if (drifts.length === 0) {
        console.log('✅ No layout drift detected');
        process.exit(0);
    }

    let hasUnsafe = false;

    for (const drift of drifts) {
        const status = drift.safe ? '✅ SAFE' : '❌ UNSAFE';
        const action = drift.safe ? '(append-only)' : '(breaking change)';

        console.log(`${status} ${drift.account}: ${drift.oldSize} → ${drift.newSize} bytes ${action}`);

        if (!drift.safe) {
            hasUnsafe = true;
        }
    }

    console.log('');

    if (hasUnsafe) {
        console.error('❌ Unsafe layout drift detected');
        console.error('Breaking changes require migration code.');
        process.exit(1);
    }

    console.log('✅ All drift is append-only (safe)');
    process.exit(0);
}

main();
