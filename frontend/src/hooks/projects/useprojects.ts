import { useMutation, useQuery, useQueryClient, type UseMutationResult, type UseQueryResult } from '@tanstack/react-query';
import { useAccount } from 'wagmi';
import type { 
  Project, 
  ProjectFromAPI, 
  CreateProjectData, 
  UpdateProjectData 
} from '@/lib/types/projects';
import { API_BASE_URL } from '@/lib/constants/apiConstants';
import { isTokenValid, getToken } from '@/lib/utils/jwt';
import { useLogin } from '@/hooks/use-login';

// Transform API response to client format
const transformProjectFromAPI = (apiProject: ProjectFromAPI): Project => {
  return {
    id: apiProject.id,
    name: apiProject.name,
    description: apiProject.description,
    status: apiProject.status,
    ownerAddress: apiProject.owner_address ?? apiProject.creator ?? '',
    createdAt: apiProject.created_at,
    updatedAt: apiProject.updated_at,
  };
};

// API functions
async function fetchProjects(): Promise<Project[]> {
  const response = await fetch(`${API_BASE_URL}/projects`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    const text = await response.text().catch(() => '');
    throw new Error(
      `Failed to fetch projects: ${response.status} ${response.statusText}${
        text ? ` - ${text}` : ''
      }`
    );
  }

  const data = (await response.json()) as ProjectFromAPI[];
  return data.map(transformProjectFromAPI);
}

async function postCreateProject(
  input: CreateProjectData,
  token: string
): Promise<Project> {
  const response = await fetch(`${API_BASE_URL}/projects`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(input),
  });
  
  if (!response.ok) {
    const text = await response.text().catch(() => '');
    throw new Error(
      `Failed to create project: ${response.status} ${response.statusText}${
        text ? ` - ${text}` : ''
      }`
    );
  }
  
  try {
    const apiProject = (await response.json()) as ProjectFromAPI;
    return transformProjectFromAPI(apiProject);
  } catch {
    throw new Error('Failed to parse project response');
  }
}

async function putUpdateProject(
  id: string,
  input: UpdateProjectData,
  token: string
): Promise<Project> {
  const response = await fetch(`${API_BASE_URL}/projects/${id}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(input),
  });
  
  if (!response.ok) {
    const text = await response.text().catch(() => '');
    throw new Error(
      `Failed to update project: ${response.status} ${response.statusText}${
        text ? ` - ${text}` : ''
      }`
    );
  }
  
  try {
    const apiProject = (await response.json()) as ProjectFromAPI;
    return transformProjectFromAPI(apiProject);
  } catch {
    throw new Error('Failed to parse project response');
  }
}

async function deleteProject(id: string, token: string): Promise<void> {
  const response = await fetch(`${API_BASE_URL}/projects/${id}`, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
  });
  
  if (!response.ok) {
    const text = await response.text().catch(() => '');
    throw new Error(
      `Failed to delete project: ${response.status} ${response.statusText}${
        text ? ` - ${text}` : ''
      }`
    );
  }
}

// React Query Hooks
export function useProjects(): UseQueryResult<Project[], Error> {
  return useQuery<Project[], Error>({
    queryKey: ['projects'],
    queryFn: fetchProjects,
  });
}

type CreateProjectVariables = {
  input: CreateProjectData;
};

export function useCreateProject(): UseMutationResult<
  Project,
  Error,
  CreateProjectVariables
> {
  const { address } = useAccount();
  const queryClient = useQueryClient();
  const { login } = useLogin();

  return useMutation<Project, Error, CreateProjectVariables>({
    mutationKey: ['create-project'],
    mutationFn: async ({ input }) => {
      if (!address) {
        throw new Error('Wallet not connected');
      }

      // Check if token is valid, if not trigger login
      if (!isTokenValid()) {
        await login();
      }

      const token = getToken();
      if (!token) {
        throw new Error('Authentication required. Please sign in.');
      }

      return postCreateProject(input, token);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['projects'] });
    },
  });
}

type UpdateProjectVariables = {
  id: string;
  input: UpdateProjectData;
};

export function useUpdateProject(): UseMutationResult<
  Project,
  Error,
  UpdateProjectVariables
> {
  const { address } = useAccount();
  const queryClient = useQueryClient();
  const { login } = useLogin();

  return useMutation<Project, Error, UpdateProjectVariables>({
    mutationKey: ['update-project'],
    mutationFn: async ({ id, input }) => {
      if (!address) {
        throw new Error('Wallet not connected');
      }

      // Check if token is valid, if not trigger login
      if (!isTokenValid()) {
        await login();
      }

      const token = getToken();
      if (!token) {
        throw new Error('Authentication required. Please sign in.');
      }

      return putUpdateProject(id, input, token);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['projects'] });
    },
  });
}

type DeleteProjectVariables = {
  id: string;
};

export function useDeleteProject(): UseMutationResult<
  void,
  Error,
  DeleteProjectVariables
> {
  const { address } = useAccount();
  const queryClient = useQueryClient();
  const { login } = useLogin();

  return useMutation<void, Error, DeleteProjectVariables>({
    mutationKey: ['delete-project'],
    mutationFn: async ({ id }) => {
      if (!address) {
        throw new Error('Wallet not connected');
      }

      // Check if token is valid, if not trigger login
      if (!isTokenValid()) {
        await login();
      }

      const token = getToken();
      if (!token) {
        throw new Error('Authentication required. Please sign in.');
      }

      return deleteProject(id, token);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['projects'] });
    },
  });
}