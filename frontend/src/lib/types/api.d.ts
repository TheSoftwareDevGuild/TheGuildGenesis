export type CreateProfileInput = {
  name: string;
  description?: string;
  avatar_url?: string;
  github_login?: string;
};

export type UpdateProfileInput = {
  name?: string;
  description?: string;
  avatar_url?: string;
  github_login?: string;
};

export type UpdateProfileResponse = unknown;

// Unknown response shape from backend; expose as unknown for consumers to refine
export type CreateProfileResponse = unknown;

export type DeleteProfileInput = {};

export type DeleteProfileResponse = unknown;
