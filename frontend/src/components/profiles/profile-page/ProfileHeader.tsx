import { User } from "lucide-react";
import { useAccount } from "wagmi";
import { useMemo } from "react";
import { useGetProfiles } from "@/hooks/profiles/use-get-profiles";
import AddressTokenBalance from "@/components/AddressTokenBalance";
import CopyAddressToClipboard from "@/components/CopyAddressToClipboard";
import { GithubIcon } from "@/components/ui/GithubIcon";

export function ProfileHeader({ address }: { address: string }) {
  const profilesQuery = useGetProfiles();

  const profile = useMemo(() => {
    const list = profilesQuery.data ?? [];
    const p = list.find(
      (x) => x.address.toLowerCase() === (address || "").toLowerCase()
    );
    return p;
  }, [profilesQuery.data, address]);

  const displayName =
    profile?.name ||
    (address ? `${address.slice(0, 6)}...${address.slice(-4)}` : "Profile");
  const displayAddress = address
    ? `${address.slice(0, 6)}...${address.slice(-4)}`
    : "";

  const { address: connectedAddress } = useAccount();
  const isOwner =
    !!connectedAddress &&
    !!address &&
    connectedAddress.toLowerCase() === address.toLowerCase();

  return (
    <header className="flex items-start gap-4">
      <div className="h-16 w-16 rounded-full bg-gray-200 flex items-center justify-center">
        <User className="h-8 w-8 text-gray-400" />
      </div>
      <div className="flex-1 min-w-0">
        <h1 className="text-2xl font-semibold flex items-center gap-3">
          {displayName}
          {isOwner && (
            <span className="text-xs bg-blue-100 text-blue-700 px-2 py-1 rounded-full font-medium">
              You
            </span>
          )}
        </h1>
        {displayAddress ? (
          <CopyAddressToClipboard 
            address={address}
            displayAddress={displayAddress}
            className="text-sm text-gray-600"
            iconSize="sm"
          />
        ) : null}
        {profile?.github_login && (
          <div className="flex items-center gap-1.5 mt-1">
            <GithubIcon className="h-4 w-4 text-gray-500" />
            <a
              href={`https://github.com/${profile.github_login}`}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm text-gray-700 hover:text-indigo-600 hover:underline"
            >
              @{profile.github_login}
            </a>
          </div>
        )}
        <AddressTokenBalance address={address as `0x${string}`} />
      </div>
    </header>
  );
}

export default ProfileHeader;