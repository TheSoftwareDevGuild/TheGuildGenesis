import React from "react";
import { Badge, BadgeCheck } from "lucide-react";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

type Badge = {
  name: string;
  description: string;
};

const HARD_CODED_BADGES: Badge[] = [
  {
    name: "Open Source Contributor",
    description: "Contributed code to The Guild Genesis repositories.",
  },
  {
    name: "Smart Contract Deployer",
    description:
      "Successfully deployed a smart contract to testnet or mainnet.",
  },
  {
    name: "Community Mentor",
    description: "Helped onboard and mentor new members of the community.",
  },
  {
    name: "Bug Hunter",
    description:
      "Reported and helped resolve significant issues or vulnerabilities.",
  },
  {
    name: "Docs Champion",
    description: "Improved documentation or tutorials for the project.",
  },
];

export function BadgesList(): React.ReactElement {
  return (
    <div className="mx-auto grid w-full max-w-5xl grid-cols-1 gap-4 p-4 sm:grid-cols-2 lg:grid-cols-3">
      {HARD_CODED_BADGES.map((badge) => (
        <Card key={badge.name}>
          <CardHeader>
            <CardTitle>
              <div className="flex items-center">
                <BadgeCheck className="h-4 w-4 mr-2" />
                {badge.name}
              </div>
            </CardTitle>
            <CardDescription>{badge.description}</CardDescription>
          </CardHeader>
          <CardContent />
        </Card>
      ))}
    </div>
  );
}

export default BadgesList;
