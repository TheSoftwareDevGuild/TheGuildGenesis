import { Search, Plus } from 'lucide-react';
import { useEffect, useState } from 'react';

export function Header() {
  const [ConnectButton, setConnectButton] = useState<React.ComponentType | null>(null);

  useEffect(() => {
    // Dynamically import ConnectButton to avoid SSR issues
    import('@rainbow-me/rainbowkit').then((module) => {
      setConnectButton(() => module.ConnectButton);
    });
  }, []);

  return (
    <header className="bg-white shadow-sm border-b">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-16">
          <div className="flex items-center">
            <h1 className="text-xl font-bold text-gray-900">
              The Guild Genesis
            </h1>
          </div>
          
          <div className="flex items-center space-x-4">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4" />
              <input
                type="text"
                placeholder="Search profiles..."
                className="pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              />
            </div>
            
            <button className="flex items-center space-x-2 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors">
              <Plus className="h-4 w-4" />
              <span>Create Profile</span>
            </button>
            
            {ConnectButton && <ConnectButton />}
          </div>
        </div>
      </div>
    </header>
  );
}
