import React, { useMemo, useState } from "react";
import { BadgeCheck } from "lucide-react";
import { BadgeCard } from "@/components/ui/BadgeCard";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useGetBadges } from "@/hooks/badges/use-get-badges";
import { HARD_CODED_BADGES } from "@/lib/constants/badgeConstants";
import type { Badge } from "@/lib/types/badges";
import { Search } from "lucide-react";
import { CreateBadgeButton } from "@/components/badges/CreateBadgeButton";
import { Input } from "../ui/input";

const badgeIcons = [
  "/badge_bug_hunter.svg",
  "/badge_community.svg",
  "/badge_documentation.svg",
  "/badge_smart_contract.svg",
  "/badge_open_source.svg",
];

export function BadgesList(): React.ReactElement {
  const { data, isLoading } = useGetBadges();
  const [searchQuery, setSearchQuery] = useState("");
  const list = (data && data.length > 0 ? data : HARD_CODED_BADGES) as Badge[];

  const filtered = useMemo(() => {
    const q = searchQuery.trim().toLowerCase();
    if (!q) return list;
    return list.filter((b) => b.name.toLowerCase().includes(q));
  }, [list, searchQuery]);

  if (isLoading) {
    return <div className="mx-auto w-full max-w-5xl p-4">Loading badges…</div>;
  }
  return (
    <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="flex gap-4 items-center pb-8">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4" />
          <Input
            type="text"
            placeholder="Search badges..."
            className="pl-10"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>

        <CreateBadgeButton />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filtered.map((badge, index) => (
          <BadgeCard
            key={badge.id ?? badge.name}
            foregroundIcon={
              <img
                src={badgeIcons[index % badgeIcons.length]}
                alt="badge icon"
                className="h-20 w-20 mr-2"
              />
            }
          >
            <CardHeader>
              <CardTitle>{badge.name}</CardTitle>
              <CardDescription>{badge.description}</CardDescription>
            </CardHeader>
            <CardContent />
          </BadgeCard>
        ))}
      </div>
    </main>
  );
}

export default BadgesList;
