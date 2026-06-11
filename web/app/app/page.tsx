export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center gap-6 p-12">
      <h1 className="text-5xl font-semibold tracking-tight">Helios Wallet</h1>
      <p className="max-w-xl text-center text-lg text-neutral-500">
        Smart-account-native wallet for the Stellar ecosystem. Passkey login, session keys, social
        recovery, sponsored transactions — built around Soroban C-addresses as the primary identity.
      </p>
      <p className="text-sm text-neutral-400">
        Early development.{" "}
        <a
          href="https://github.com/heliosproto/heliosproto"
          className="underline underline-offset-4 hover:text-neutral-200"
        >
          Source on GitHub
        </a>
        .
      </p>
    </main>
  );
}
