import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { ProfileCard } from '../components/ProfileCard';

describe('ProfileCard', () => {
  const mockProfile = {
    address: '0x1234567890123456789012345678901234567890',
    name: 'Alice Developer',
    description: 'Full-stack developer passionate about Web3 and Rust',
    avatar: 'https://example.com/avatar.jpg',
    badgeCount: 3,
    badges: [
      {
        id: '1',
        name: 'Rust',
        description: 'Rust programming',
        issuer: '0xabcd1234567890123456789012345678901234567890'
      },
      {
        id: '2',
        name: 'React',
        description: 'React development',
        issuer: '0xefgh1234567890123456789012345678901234567890'
      }
    ]
  };

  it('renders profile information correctly', () => {
    render(<ProfileCard {...mockProfile} />);
    
    expect(screen.getByText('Alice Developer')).toBeInTheDocument();
    expect(screen.getByText('0x1234...7890')).toBeInTheDocument();
    expect(screen.getByText('Full-stack developer passionate about Web3 and Rust')).toBeInTheDocument();
    expect(screen.getByText('3 badges')).toBeInTheDocument();
  });

  it('renders badges correctly', () => {
    render(<ProfileCard {...mockProfile} />);
    
    expect(screen.getByText('Rust')).toBeInTheDocument();
    expect(screen.getByText('React')).toBeInTheDocument();
  });

  it('renders with minimal data', () => {
    const minimalProfile = {
      address: '0x1234567890123456789012345678901234567890',
      badgeCount: 0,
      badges: []
    };
    
    render(<ProfileCard {...minimalProfile} />);
    
    // Check that the address appears in both places
    const addressElements = screen.getAllByText('0x1234...7890');
    expect(addressElements).toHaveLength(2); // One in name, one in address
    expect(screen.getByText('0 badges')).toBeInTheDocument();
  });

  it('shows truncated address for long addresses', () => {
    const longAddress = '0x1234567890123456789012345678901234567890';
    render(<ProfileCard {...mockProfile} address={longAddress} />);
    
    expect(screen.getByText('0x1234...7890')).toBeInTheDocument();
  });
});
