-- # Entity schema.

-- Create `users` table.
create table if not exists users (
    id integer primary key autoincrement,
    username text not null unique,
    grade integer not null,
    realname text not null,
    password text not null,
    bracket text,
    score integer default 0
);

-- Create `groups` table.
create table if not exists groups (
    id integer primary key autoincrement,
    name text not null unique
);

-- Create `permissions` table.
create table if not exists permissions (
    id integer primary key autoincrement,
    name text not null unique
);


create table if not exists admindata (
    bracket text not null
);
    

-- # Join tables.

-- Create `users_groups` table for many-to-many relationships between users and groups.
create table if not exists users_groups (
    user_id integer references users(id),
    group_id integer references groups(id),
    primary key (user_id, group_id)
);

-- Create `groups_permissions` table for many-to-many relationships between groups and permissions.
create table if not exists groups_permissions (
    group_id integer references groups(id),
    permission_id integer references permissions(id),
    primary key (group_id, permission_id)
);


-- # Fixture hydration.

insert into admindata (bracket)
values ('{
    "Round 1": ["Example", "Example", "Example"],
    "Round 2": ["Example", "Example"],
    "Round 3": ["Example"],
    "Round 4": ["Example"],
    "Round 5": ["Example"],
    "Champion": null,
    "Wild Card": null
}');

-- Insert "ferris" user.
insert into users (username, password, grade, realname)
values (
    'ferris',
    '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw',
    0,
    'ferris'
);

-- Insert "admin" user.
insert into users (username, password, grade, realname)
values (
    'admin',
    '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw',
    0,
    'admin'
);

-- Insert "users" and "superusers" groups.
insert into groups (name) values ('users');
insert into groups (name) values ('superusers');

-- Insert individual permissions.
insert into permissions (name) values ('protected.read');
insert into permissions (name) values ('restricted.read');

-- Insert group permissions.
insert into groups_permissions (group_id, permission_id)
values (
    (select id from groups where name = 'users'),
    (select id from permissions where name = 'protected.read')
), (
    (select id from groups where name = 'superusers'),
    (select id from permissions where name = 'restricted.read')
);

-- Insert users into groups.
insert into users_groups (user_id, group_id)
values (
    (select id from users where username = 'ferris'),
    (select id from groups where name = 'users')
), (
    (select id from users where username = 'admin'),
    (select id from groups where name = 'users')
), (
    (select id from users where username = 'admin'),
    (select id from groups where name = 'superusers')
);
