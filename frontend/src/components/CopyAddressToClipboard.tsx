import { useState } from "react";
import { Copy, Check } from "lucide-react";
import { cn } from "@/lib/utils";

interface CopyAddressToClipboardProps {
  address: string;
  displayAddress?: string;
  className?: string;
  iconSize?: "sm" | "md";
  showFeedback?: boolean;
}

export function CopyAddressToClipboard({
  address,
  displayAddress,
  className,
  iconSize = "sm",
  showFeedback = true,
}: CopyAddressToClipboardProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    
    try {
      await navigator.clipboard.writeText(address);
      if (showFeedback) {
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
      }
    } catch (err) {
      console.error("Failed to copy address:", err);
    }
  };

  const iconClasses = cn(
    "transition-colors hover:text-gray-700 cursor-pointer",
    iconSize === "sm" ? "h-3.5 w-3.5" : "h-4 w-4",
    copied ? "text-green-600" : "text-gray-500"
  );

  return (
    <div className={cn("inline-flex items-center gap-2", className)}>
      {displayAddress && (
        <span className="font-mono">{displayAddress}</span>
      )}
      <button
        onClick={handleCopy}
        className="inline-flex items-center justify-center p-1 rounded hover:bg-gray-100 transition-colors"
        title={copied ? "Copied!" : "Copy full address to clipboard"}
        aria-label={copied ? "Address copied" : "Copy address to clipboard"}
      >
        {copied ? (
          <Check className={iconClasses} />
        ) : (
          <Copy className={iconClasses} />
        )}
      </button>
    </div>
  );
}

export default CopyAddressToClipboard;
