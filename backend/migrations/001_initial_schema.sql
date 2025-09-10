CREATE TABLE profiles (
    address VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    description TEXT,
    avatar_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
