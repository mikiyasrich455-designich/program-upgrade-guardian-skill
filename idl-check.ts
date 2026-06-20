import { Idl } from "@coral-xyz/anchor";
import { readFileSync } from "fs";

const OLD_IDL_PATH = process.env.OLD_IDL_PATH || "idl/onchain.json";
const NEW_IDL_PATH = process.env.NEW_IDL_PATH || "target/idl/my_program.json";

interface IdlCheckResult {
  passed: boolean;
  errors: string[];
  warnings: string[];
}

function checkIdlCompatibility(oldIdl: Idl, newIdl: Idl): IdlCheckResult {
  const errors: string[] = [];
  const warnings: string[] = [];

  for (const oldAcc of oldIdl.accounts ?? []) {
    const newAcc = newIdl.accounts?.find((a) => a.name === oldAcc.name);

    if (!newAcc) {
      errors.push(`BLOCK: Account '${oldAcc.name}' removed from IDL`);
      continue;
    }

    const oldFields = (oldAcc.type as any).fields.map((f: any) => ({
      name: f.name,
      type: JSON.stringify(f.type),
    }));
    const newFields = (newAcc.type as any).fields.map((f: any) => ({
      name: f.name,
      type: JSON.stringify(f.type),
    }));

    for (let i = 0; i < oldFields.length; i++) {
      if (i >= newFields.length) {
        errors.push(`BLOCK: Field '${oldFields[i].name}' missing from '${oldAcc.name}' (would shift indices)`);
        continue;
      }
      if (oldFields[i].name !== newFields[i].name) {
        errors.push(`BLOCK: Field order changed in '${oldAcc.name}'. Expected '${oldFields[i].name}' at index ${i}, found '${newFields[i].name}'`);
      }
      if (oldFields[i].type !== newFields[i].type) {
        errors.push(`BLOCK: Field '${oldFields[i].name}' type changed in '${oldAcc.name}'. Old: ${oldFields[i].type}, New: ${newFields[i].type}`);
      }
    }

    for (const oldField of oldFields) {
      if (!newFields.some((f: any) => f.name === oldField.name)) {
        errors.push(`BLOCK: Field '${oldField.name}' removed from '${oldAcc.name}'`);
      }
    }

    for (const newField of newFields) {
      if (!oldFields.some((f: any) => f.name === newField.name)) {
        warnings.push(`INFO: New field '${newField.name}' added to '${oldAcc.name}' at index ${newFields.indexOf(newField)}`);
      }
    }
  }

  for (const oldType of oldIdl.types ?? []) {
    if (oldType.type.kind !== "enum") continue;

    const newType = newIdl.types?.find((t) => t.name === oldType.name);
    if (!newType) {
      errors.push(`BLOCK: Enum '${oldType.name}' removed from IDL`);
      continue;
    }

    const oldVariants = (oldType.type as any).variants.map((v: any) => v.name);
    const newVariants = (newType.type as any).variants.map((v: any) => v.name);

    for (let i = 0; i < oldVariants.length; i++) {
      if (i >= newVariants.length) {
        errors.push(`BLOCK: Enum variant '${oldVariants[i]}' missing from '${oldType.name}'`);
        continue;
      }
      if (oldVariants[i] !== newVariants[i]) {
        errors.push(`BLOCK: Enum variant order changed in '${oldType.name}'. Expected '${oldVariants[i]}' at index ${i}, found '${newVariants[i]}'`);
      }
    }

    for (const v of oldVariants) {
      if (!newVariants.includes(v)) {
        errors.push(`BLOCK: Enum variant '${v}' removed from '${oldType.name}'`);
      }
    }
  }

  for (const oldIx of oldIdl.instructions ?? []) {
    const newIx = newIdl.instructions?.find((i) => i.name === oldIx.name);
    if (!newIx) {
      warnings.push(`INFO: Instruction '${oldIx.name}' removed (backward compat risk)`);
    }
  }

  return { passed: errors.length === 0, errors, warnings };
}

const oldIdl: Idl = JSON.parse(readFileSync(OLD_IDL_PATH, "utf-8"));
const newIdl: Idl = JSON.parse(readFileSync(NEW_IDL_PATH, "utf-8"));

const result = checkIdlCompatibility(oldIdl, newIdl);

console.log("\n═══════════════════════════════════════════════════");
console.log("  IDL Compatibility Check");
console.log("═══════════════════════════════════════════════════\n");

if (result.warnings.length > 0) {
  console.log("⚠️  Warnings:");
  result.warnings.forEach((w) => console.log(`   ${w}`));
  console.log();
}

if (result.errors.length > 0) {
  console.log("❌ BLOCKING ERRORS:");
  result.errors.forEach((e) => console.log(`   ${e}`));
  console.log("\n⛔ Upgrade BLOCKED. Fix errors before deploying.\n");
  process.exit(1);
} else {
  console.log("✅ IDL compatible: append-only changes detected.\n");
  process.exit(0);
}
