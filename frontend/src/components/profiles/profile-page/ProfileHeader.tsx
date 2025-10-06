import { User } from "lucide-react";
import { useAccount } from "wagmi";
import { useMemo } from "react";
import { useGetProfiles } from "@/hooks/profiles/use-get-profiles";
import AddressTokenBalance from "@/components/AddressTokenBalance";
import CopyAddressToClipboard from "@/components/CopyAddressToClipboard";

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
        {profile?.github_handle && (
          <div className="flex items-center gap-1.5 mt-1">
            <svg
              className="h-4 w-4 text-gray-500"
              viewBox="0 0 24 24"
              fill="currentColor"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.17 6.839 9.49.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.463-1.11-1.463-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.578 9.578 0 0112 6.836c.85.004 1.705.114 2.504.336 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.138 20.167 22 16.418 22 12c0-5.523-4.477-10-10-10z" />
            </svg>
            <a
              href={`https://github.com/${profile.github_handle}`}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm text-gray-700 hover:text-indigo-600 hover:underline"
            >
              @{profile.github_handle}
            </a>
          </div>
        )}
        <AddressTokenBalance address={address as `0x${string}`} />
      </div>
    </header>
  );
}

export default ProfileHeader;