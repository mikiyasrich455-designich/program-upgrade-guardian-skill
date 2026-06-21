import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DemoProgram } from "../target/types/demo_program";

// ═══════════════════════════════════════════════════════════════════════
// Program Upgrade Guardian — TypeScript Client Demo
// ═══════════════════════════════════════════════════════════════════════
// Usage: ts-node app/client.ts
// ═══════════════════════════════════════════════════════════════════════

async function main() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.DemoProgram as Program<DemoProgram>;
    const user = provider.wallet;

    console.log("Program ID:", program.programId.toBase58());
    console.log("User:", user.publicKey.toBase58());

    // 1. Initialize v2 profile
    const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user_profile"), user.publicKey.toBuffer()],
        program.programId
    );

    console.log("\n1. Initializing v2 profile...");
    await program.methods
        .initializeProfile("alice")
        .accounts({
            userProfile: profilePda,
            authority: user.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

    let profile = await program.account.userProfile.fetch(profilePda);
    console.log("   Version:", profile.version);
    console.log("   Name:", profile.name);
    console.log("   Bio:", profile.bio);
    console.log("   Reputation:", profile.reputationScore.toString());

    // 2. Update bio (v2 feature)
    console.log("\n2. Updating bio...");
    await program.methods
        .updateBio("Solana builder")
        .accounts({
            userProfile: profilePda,
            owner: user.publicKey,
        })
        .rpc();

    profile = await program.account.userProfile.fetch(profilePda);
    console.log("   Bio:", profile.bio);

    console.log("\n✅ Demo complete!");
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
