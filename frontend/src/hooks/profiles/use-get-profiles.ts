import type { ProfileFromAPI } from "@/lib/types/profiles";
import { useQuery, type UseQueryResult } from "@tanstack/react-query";
import { API_BASE_URL } from "@/lib/constants/apiConstants";

async function fetchProfiles(): Promise<ProfileFromAPI[]> {
  const response = await fetch(`${API_BASE_URL}/profiles`, {
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

  return (await response.json()) as ProfileFromAPI[];
}

export function useGetProfiles(): UseQueryResult<ProfileFromAPI[], Error> {
  return useQuery<ProfileFromAPI[], Error>({
    queryKey: ["profiles"],
    queryFn: fetchProfiles,
  });
}
