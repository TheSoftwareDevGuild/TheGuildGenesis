export type Badge = {
  name: string;
  description: string;
  creator: string;
  voteScore: number;
  pointers?: string[]; // optional for backward compatibility
};
