import { useState } from "react";
import { useAccount } from "wagmi";
import { useLogin } from "@/hooks/use-login";
import { isTokenValid, clearToken } from "@/lib/utils/jwt";
import { Button } from "@/components/ui/button";
import { LogIn, LogOut } from "lucide-react";

export function LoginButton() {
  const { address, isConnected } = useAccount();
  const { login, isLoading, error } = useLogin();
  const [isLoggedIn, setIsLoggedIn] = useState(isTokenValid());

  const handleLogin = async () => {
    try {
      await login();
      setIsLoggedIn(true);
    } catch (err) {
      console.error("Login failed:", err);
    }
  };

  const handleLogout = () => {
    clearToken();
    setIsLoggedIn(false);
  };

  // Show nothing if wallet not connected
  if (!isConnected || !address) {
    return null;
  }

  if (isLoggedIn) {
    return (
      <Button
        variant="outline"
        size="sm"
        onClick={handleLogout}
        className="flex items-center gap-2"
      >
        <LogOut className="h-4 w-4" />
        <span>Logout</span>
      </Button>
    );
  }

  return (
    <Button
      size="sm"
      onClick={handleLogin}
      disabled={isLoading}
      className="flex items-center gap-2"
    >
      <LogIn className="h-4 w-4" />
      <span>{isLoading ? "Signing..." : "Sign In"}</span>
    </Button>
  );
}
