#!/usr/bin/env node
import { Command } from "commander";
import { Presets } from "./index";

const program = new Command();

program
  .name("sss-token")
  .description("CLI for Solana Stablecoin Standard Operations")
  .version("1.0.0");

program
  .command("init")
  .description("Initialize a new stablecoin using SSS presets")
  .option("--preset <type>", "Preset to use (sss-1 or sss-2)")
  .option("--name <name>", "Token name")
  .option("--symbol <symbol>", "Token symbol")
  .action((options) => {
    if (!options.preset || !Object.values(Presets).includes(options.preset)) {
      console.error("❌ Invalid preset. Use 'sss-1' or 'sss-2'.");
      process.exit(1);
    }
    console.log(`🚀 Initializing stablecoin: ${options.name} (${options.symbol})`);
    console.log(`⚙️  Applying preset rules for: ${options.preset.toUpperCase()}`);
    console.log(`🔗 Connecting to Anchor Program...`);
    // Tässä kutsuttaisiin SolanaStablecoin.create()
    console.log(`✅ Success! Mint Address: 8xTest... (mock)`);
  });

program
  .command("blacklist")
  .description("Manage SSS-2 compliance blacklist")
  .argument("<action>", "add or remove")
  .argument("<address>", "Target wallet address")
  .option("--reason <reason>", "Reason for blacklisting")
  .action((action, address, options) => {
    if (action === "add") {
      console.log(`🔒 Adding ${address} to blacklist.`);
      if (options.reason) console.log(`📝 Reason logged: "${options.reason}"`);
      // Kutsuttaisiin stable.compliance.blacklistAdd()
    }
  });

program.parse();
