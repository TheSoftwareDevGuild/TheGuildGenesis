import { User } from "lucide-react";
import { useAccount } from "wagmi";
import AddressTokenBalance from "@/components/AddressTokenBalance";
import CopyAddressToClipboard from "@/components/CopyAddressToClipboard";
import { GithubIcon } from "@/components/ui/GithubIcon";
import { XIcon } from "@/components/ui/XIcon";

// Helper function to extract handle or full URL from LinkedIn input
const parseLinkedinAccount = (
  account: string
): { displayHandle: string; profileUrl: string } => {
  if (!account) return { displayHandle: "", profileUrl: "" };

  // If it's already a full URL, extract the handle and use it as-is
  if (account.startsWith("http")) {
    // Already a full URL, just ensure it's properly formatted
    const url =
      account.endsWith("/") || account.endsWith("recruit")
        ? account
        : account + "/";
    const handle =
      account.match(/\/in\/([^/?]+)/)?.[1] || account.split("/").pop() || "";
    return {
      displayHandle: handle || "LinkedIn",
      profileUrl: url,
    };
  }

  // It's just a handle, construct the full URL
  return {
    displayHandle: account,
    profileUrl: `https://www.linkedin.com/in/${account}/`,
  };
};

interface ProfileHeaderProps {
  address: string;
  name?: string;
  description?: string;
  githubLogin?: string;
  twitterHandle?: string;
  linkedinAccount?: string;
}

export function ProfileHeader({
  address,
  name,
  githubLogin,
  twitterHandle,
  linkedinAccount,
}: ProfileHeaderProps) {
  const displayName = name || (address ? `${address.slice(0, 6)}...${address.slice(-4)}` : "Profile");
  const displayAddress = address ? `${address.slice(0, 6)}...${address.slice(-4)}` : "";

  const { address: connectedAddress } = useAccount();
  const isOwner =
    !!connectedAddress &&
    !!address &&
    connectedAddress.toLowerCase() === address.toLowerCase();

  const linkedinData =
    linkedinAccount && parseLinkedinAccount(linkedinAccount);

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
        {githubLogin && (
          <div className="flex items-center gap-1.5 mt-1">
            <GithubIcon className="h-4 w-4 text-gray-500" />
            <a
              href={`https://github.com/${githubLogin}`}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm text-gray-700 hover:text-indigo-600 hover:underline"
            >
              @{githubLogin}
            </a>
          </div>
        )}
        {twitterHandle && (
          <div className="flex items-center gap-1.5 mt-1">
            <XIcon className="h-4 w-4 text-gray-500" />
            <a
              href={`https://x.com/${twitterHandle}`}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm text-gray-700 hover:text-indigo-600 hover:underline"
            >
              @{twitterHandle}
            </a>
          </div>
        )}
        {linkedinData && (
          <div className="flex items-center gap-1.5 mt-1">
            <a
              href={linkedinData.profileUrl}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm text-gray-700 hover:text-indigo-600 hover:underline"
            >
              🔗 {linkedinData.displayHandle}
            </a>
          </div>
        )}
        <AddressTokenBalance address={address as `0x${string}`} />
      </div>
    </header>
  );
}

export default ProfileHeader;