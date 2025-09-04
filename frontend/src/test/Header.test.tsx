import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { Header } from '../components/Header';

// Mock RainbowKit components
vi.mock('@rainbow-me/rainbowkit', () => ({
  ConnectButton: () => <div data-testid="connect-button">Connect Wallet</div>
}));

describe('Header', () => {
  it('renders the title', () => {
    render(<Header />);
    expect(screen.getByText('The Guild Genesis')).toBeInTheDocument();
  });

  it('renders search input', () => {
    render(<Header />);
    expect(screen.getByPlaceholderText('Search profiles...')).toBeInTheDocument();
  });

  it('renders create profile button', () => {
    render(<Header />);
    expect(screen.getByText('Create Profile')).toBeInTheDocument();
  });

  it('renders connect wallet button', () => {
    render(<Header />);
    expect(screen.getByTestId('connect-button')).toBeInTheDocument();
  });
});

