export const payForStream = async () => {
    const anchor = window.anchor;
    if (!anchor) throw new Error("Anchor library not loaded");
  
    const { solana } = window;
    if (!solana || !solana.isPhantom) throw new Error("Phantom wallet not detected");
  
    const connection = new anchor.web3.Connection("http://localhost:8899");
    const provider = new anchor.AnchorProvider(
      connection,
      solana,
      { commitment: "confirmed" }
    );
    anchor.setProvider(provider);
  
    const programId = new anchor.web3.PublicKey("8Htw3BkK7hopp7L7BeoD7NkecGA6QMS6hzp1dppLfuje");
    const idl = await anchor.Program.fetchIdl(programId, provider);
    if (!idl) throw new Error("Failed to fetch IDL");
    const program = new anchor.Program(idl, programId, provider);
  
    const amount = 100_000_000; // 0.1 SOL
    const [entitlementPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("entitlement"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
  
    await program.methods
      .initializePayment(amount)
      .accounts({
        entitlement: entitlementPda,
        user: provider.wallet.publicKey,
        vault: new anchor.web3.PublicKey("33cb5TudLKVHvqUKn96QBbXbvjCGTHB7aLxrhG3HduyJ"), // Replace with vault key
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  
    console.log("Payment initialized!");
  };