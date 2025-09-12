import { useQuery, type UseQueryResult } from "@tanstack/react-query";

export type Profile = {
  address: string;
  name?: string;
  description?: string;
  avatar_url?: string;
  created_at?: string;
  updated_at?: string;
};

const API_BASE_URL: string =
  import.meta.env.PUBLIC_API_URL || "http://0.0.0.0:3001";

async function fetchProfiles(): Promise<Profile[]> {
  const response = await fetch(`${API_BASE_URL}/profiles/`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });

  if (!response.ok) {
    const text = await response.text().catch(() => "");
    throw new Error(
      `Failed to fetch profiles: ${response.status} ${response.statusText}${
        text ? ` - ${text}` : ""
      }`
    );
  }

  return (await response.json()) as Profile[];
}

export function useGetProfiles(): UseQueryResult<Profile[], Error> {
  return useQuery<Profile[], Error>({
    queryKey: ["profiles"],
    queryFn: fetchProfiles,
  });
}
