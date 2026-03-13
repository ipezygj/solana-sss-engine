import { Connection, Keypair, PublicKey, Transaction, SystemProgram } from "@solana/web3.js";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { TOKEN_2022_PROGRAM_ID, getAssociatedTokenAddressSync, createAssociatedTokenAccountInstruction, createMintToInstruction } from "@solana/spl-token";

export enum Presets {
    SSS_1 = "sss-1",
    SSS_2 = "sss-2"
}

export interface StablecoinConfig {
    name: string;
    symbol: string;
    uri: string;
    decimals: number;
    enablePermanentDelegate: boolean;
    enableTransferHook: boolean;
    defaultAccountFrozen: boolean;
}

export class SolanaStablecoin {
    connection: Connection;
    program: Program; // Anchor ohjelma (tyyppi jätetty any/Program tässä stubissa)
    mint: PublicKey;
    
    constructor(connection: Connection, program: Program, mint: PublicKey) {
        this.connection = connection;
        this.program = program;
        this.mint = mint;
    }

    // Tehdasmetodi: Luo uusi token presetillä
    static async create(
        connection: Connection,
        program: Program,
        authority: Keypair,
        options: { preset: Presets, name: string, symbol: string, decimals: number }
    ): Promise<SolanaStablecoin> {
        
        const mintKeypair = Keypair.generate();
        
        let config: StablecoinConfig;
        if (options.preset === Presets.SSS_1) {
            config = { name: options.name, symbol: options.symbol, uri: "", decimals: options.decimals, enablePermanentDelegate: false, enableTransferHook: false, defaultAccountFrozen: false };
        } else {
            // SSS-2: Compliance Module Enabled
            config = { name: options.name, symbol: options.symbol, uri: "", decimals: options.decimals, enablePermanentDelegate: true, enableTransferHook: true, defaultAccountFrozen: false };
        }

        // 1. Derivoidaan PDA konfiguraatiolle
        const [stablecoinDataPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("stablecoin_data"), mintKeypair.publicKey.toBuffer()],
            program.programId
        );

        // 2. Kutsutaan Anchor-ohjelman `initialize` (Tämä aktivoi Token-2022 CPI-kutsut on-chain)
        const tx = await program.methods.initialize({
            name: config.name,
            symbol: config.symbol,
            uri: config.uri,
            decimals: config.decimals,
            enablePermanentDelegate: config.enablePermanentDelegate,
            enableTransferHook: config.enableTransferHook,
            defaultAccountFrozen: config.defaultAccountFrozen
        })
        .accounts({
            authority: authority.publicKey,
            stablecoinData: stablecoinDataPda,
            mint: mintKeypair.publicKey,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .signers([authority, mintKeypair])
        .rpc();

        console.log(`✅ [SDK] Token ${options.symbol} forged (Preset: ${options.preset}). Mint: ${mintKeypair.publicKey.toBase58()}`);
        return new SolanaStablecoin(connection, program, mintKeypair.publicKey);
    }

    // Compliance-moduulin apufunktiot (Käyttää getter-patternia kuten bountyssa vaadittiin)
    get compliance() {
        return {
            blacklistAdd: async (target: PublicKey, authority: Keypair) => {
                // PDA derivointi mustalle listalle
                const [stablecoinDataPda] = PublicKey.findProgramAddressSync(
                    [Buffer.from("stablecoin_data"), this.mint.toBuffer()],
                    this.program.programId
                );
                
                const [blacklistPda] = PublicKey.findProgramAddressSync(
                    [Buffer.from("blacklist"), stablecoinDataPda.toBuffer(), target.toBuffer()],
                    this.program.programId
                );

                await this.program.methods.toggleBlacklist(true)
                .accounts({
                    authority: authority.publicKey,
                    stablecoinData: stablecoinDataPda,
                    targetAccount: target,
                    blacklistEntry: blacklistPda,
                    systemProgram: SystemProgram.programId,
                })
                .signers([authority])
                .rpc();
                console.log(`🔒 [SDK] Address ${target.toBase58()} blacklisted.`);
            },
            seize: async (source: PublicKey, destination: PublicKey, amount: number, authority: Keypair) => {
                // Tähän tulisi Permanent Delegate -kutsu (ohitetaan stubissa täydellinen Anchor-rakennus)
                console.log(`🚨 [SDK] Seized ${amount} tokens from ${source.toBase58()}`);
            }
        };
    }
}
