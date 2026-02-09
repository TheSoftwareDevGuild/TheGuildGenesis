type ProfileAttestation = {
  id: string;
  badgeName: string;
  justification: string;
  issuer: string;
};

export type Profile = {
  address: string;
  name: string;
  description: string;
  githubLogin?: string;
  twitterHandle?: string;
  attestationCount: number;
  attestations: ProfileAttestation[];
};

export type ProfileFromAPI = {
  address: string;
  name?: string;
  description?: string;
  avatar_url?: string;
  github_login?: string;
  twitter_handle?: string;
  created_at?: string;
  updated_at?: string;
};