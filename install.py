#!/usr/bin/env python3
"""
Program Upgrade Guardian - Setup & Dependency Checker
A simple, readable Python script that checks your environment.
No hidden commands, no system modifications.
"""

import subprocess
import sys
import os

# Colors for terminal output
RED = '\033[91m'
GREEN = '\033[92m'
YELLOW = '\033[93m'
BLUE = '\033[94m'
BOLD = '\033[1m'
RESET = '\033[0m'

def print_header(text):
    print(f"\n{BOLD}{'='*50}{RESET}")
    print(f"{BOLD}{text}{RESET}")
    print(f"{BOLD}{'='*50}{RESET}\n")

def print_ok(text):
    print(f"  {GREEN}✓{RESET} {text}")

def print_warn(text):
    print(f"  {YELLOW}⚠{RESET} {text}")

def print_error(text):
    print(f"  {RED}✗{RESET} {text}")

def run_command(cmd):
    """Run a command and return (success, output)"""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, timeout=10)
        return result.returncode == 0, result.stdout.strip()
    except Exception as e:
        return False, str(e)

def check_tool(name, min_version, version_cmd):
    """Check if a tool is installed and get its version"""
    print(f"Checking {name}...", end=" ")
    success, output = run_command(f"command -v {name}")
    if not success or not output:
        print(f"{RED}NOT FOUND{RESET}")
        return False, None

    # Try to get version
    v_success, v_output = run_command(version_cmd)
    if v_success and v_output:
        print(f"{GREEN}OK{RESET} ({v_output})")
        return True, v_output
    else:
        print(f"{GREEN}OK{RESET} (version unknown)")
        return True, "unknown"

def main():
    print_header("Program Upgrade Guardian - Setup Check")

    errors = 0
    warnings = 0

    # Check core tools
    print(f"{BOLD}Core Tools:{RESET}")

    tools = [
        ("solana", "1.18.0", "solana --version"),
        ("anchor", "0.30.0", "anchor --version"),
        ("cargo", "1.75.0", "cargo --version"),
        ("rustc", "1.75.0", "rustc --version"),
    ]

    for name, min_ver, ver_cmd in tools:
        ok, version = check_tool(name, min_ver, ver_cmd)
        if not ok:
            errors += 1
            print(f"    Install: See docs at https://solana.com/docs/intro/installation")

    # Check optional tools
    print(f"\n{BOLD}Optional Tools:{RESET}")

    optional = [
        ("sqd", "Squads CLI", "sqd --version"),
        ("node", "Node.js 20+", "node --version"),
        ("npm", "npm", "npm --version"),
    ]

    for name, desc, ver_cmd in optional:
        ok, version = check_tool(desc, "latest", ver_cmd)
        if not ok:
            warnings += 1

    # Check Surfpool
    ok, _ = check_tool("surfpool", "latest", "surfpool --version")
    if not ok:
        warnings += 1
        print_warn("Surfpool not found (install: cargo install surfpool-cli)")

    # Check Solana config
    print(f"\n{BOLD}Solana Configuration:{RESET}")
    success, rpc_url = run_command("solana config get | grep 'RPC URL'")
    if success:
        rpc = rpc_url.split()[-1] if rpc_url.split() else "unknown"
        if "mainnet" in rpc.lower():
            print_error(f"Currently on MAINNET: {rpc}")
            print(f"    {YELLOW}Switch to devnet: solana config set --url https://api.devnet.solana.com{RESET}")
            warnings += 1
        else:
            print_ok(f"Cluster: {rpc}")
    else:
        print_warn("Could not determine Solana cluster")

    # Check keypair
    success, keypair = run_command("solana config get | grep 'Keypair Path'")
    if success:
        kp_path = keypair.split()[-1] if keypair.split() else None
        if kp_path and os.path.exists(os.path.expanduser(kp_path)):
            s2, pubkey = run_command("solana address")
            if s2:
                print_ok(f"Keypair: {pubkey}")
                s3, balance = run_command("solana balance")
                if s3:
                    print(f"    Balance: {balance}")
            else:
                print_warn("Could not read keypair")
        else:
            print_error("Keypair not found")
            errors += 1
    else:
        print_warn("No keypair configured")
        errors += 1

    # Check for .squads-config
    print(f"\n{BOLD}Multisig Configuration:{RESET}")
    if os.path.exists(".squads-config"):
        print_ok(".squads-config found")
    else:
        print_warn(".squads-config not found (create with MULTISIG_PUBKEY=...)")
        warnings += 1

    # Check Cargo.toml for litesvm
    print(f"\n{BOLD}Testing Setup:{RESET}")
    if os.path.exists("Cargo.toml"):
        with open("Cargo.toml", "r") as f:
            content = f.read()
            if "litesvm" in content:
                print_ok("LiteSVM configured in Cargo.toml")
            else:
                print_warn("LiteSVM not in Cargo.toml (add: litesvm = \"0.6\" to [dev-dependencies])")
                warnings += 1
    else:
        print_warn("No Cargo.toml found in current directory")
        warnings += 1

    # Summary
    print_header("Summary")

    if errors > 0:
        print(f"{RED}{BOLD}Result: {errors} error(s), {warnings} warning(s){RESET}")
        print(f"{RED}Fix errors before proceeding with any upgrade.{RESET}")
        return 1
    elif warnings > 0:
        print(f"{YELLOW}{BOLD}Result: 0 errors, {warnings} warning(s){RESET}")
        print(f"{YELLOW}You can proceed, but address warnings for full safety.{RESET}")
        return 0
    else:
        print(f"{GREEN}{BOLD}Result: All checks passed! Guardian is ready.{RESET}")
        return 0

if __name__ == "__main__":
    sys.exit(main())
