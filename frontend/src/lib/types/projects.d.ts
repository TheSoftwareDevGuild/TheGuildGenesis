export type Project = {
  id: string;
  name: string;
  description: string;
  ownerAddress: string;
  createdAt: string;
  updatedAt: string;
};

export type ProjectFromAPI = {
  id: string;
  name: string;
  description: string;
  status: string;
  creator: string;
  owner_address: string; 
  created_at: string;
  updated_at: string;
};

export type CreateProjectData = {
  name: string;
  description: string;
};

export type UpdateProjectData = {
  name: string;
  description: string;
};