import { AppWrapper } from "@/components/AppWrapper";

export default function LeaderboardContent() {
  return (
    <AppWrapper>
      <section className="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8 py-10 space-y-8">
        <div className="flex items-center justify-between">
          <h1 className="text-3xl font-bold tracking-tight">LeaderBoard</h1>
          <a
            href="https://discord.gg/axCqT23Xhj"
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center rounded-md bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            Join our Discord
          </a>
        </div>
        {/* Leaderboard content goes here */}
      </section>
    </AppWrapper>
  )
}

