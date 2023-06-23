DO $$
BEGIN
    IF 
        NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'ACCESS_TYPE')
    THEN
        -- Define access type. resticted_access means that user has access only to the owned resource
        CREATE TYPE ACCESS_TYPE AS ENUM ('no_access', 'unrestricted_access', 'restricted_access');
    END IF;

    IF 
        NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'USER_ROLE')
    THEN
        -- Define user role type.
        CREATE TYPE USER_ROLE AS ENUM ('SuperAdmin', 'Admin', 'User');
    END IF;

    IF 
        NOT EXISTS (SELECT FROM information_schema.tables WHERE  table_schema = 'public' AND table_name   = 'roles')
    THEN 
        -- Create roles table
        CREATE TABLE roles (
            id SERIAL PRIMARY KEY,
            role USER_ROLE NOT NULL, -- Name of the role
            delete_user ACCESS_TYPE, -- Starting here columns with route names, values are access types
            get_user ACCESS_TYPE,
            update_user ACCESS_TYPE
        );

        -- Create initial roles
        INSERT INTO roles (role, delete_user, get_user, update_user)
        VALUES 
            ('SuperAdmin', 'unrestricted_access', 'unrestricted_access', 'unrestricted_access'),
            ('Admin', 'unrestricted_access', 'unrestricted_access', 'restricted_access'),
            ('User', 'restricted_access', 'restricted_access', 'restricted_access');

        -- Define relation between roles and users
        ALTER TABLE users 
        ADD COLUMN role_id INTEGER REFERENCES roles(id);

        -- Set roles of existing users to User
        UPDATE users 
        SET role_id = (SELECT id FROM roles WHERE role = 'User')
        WHERE role_id IS NULL;

        -- Add NOT NULL constraint to role_id column
        ALTER TABLE users ALTER COLUMN role_id SET NOT NULL;
    END IF;
END
$$;