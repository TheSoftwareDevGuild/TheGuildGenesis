
export type CreateProfileInput = {
  name: string;
  description?: string;
  avatar_url?: string;
  github_handle?: string; 
  siweMessage: string; 
};

export type UpdateProfileInput = {
  name?: string;
  description?: string;
  avatar_url?: string;
  github_handle?: string; 
  siweMessage: string;
};

export type UpdateProfileResponse = unknown;

// Unknown response shape from backend; expose as unknown for consumers to refine
export type CreateProfileResponse = unknown;

export type DeleteProfileInput = {
  siweMessage: string;
};

export type DeleteProfileResponse = unknown;