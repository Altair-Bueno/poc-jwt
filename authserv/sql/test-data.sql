insert into "role_table"(name) values ('ADMIN');
insert into "role_table"(name) values ('USER');
insert into "role_table"(name) values ('ANALYST');

-- username: admin
-- password: password
insert into "user_table"(id, username, password, deleted_at) VALUES ('223bfb67-2e22-4ce3-a010-bfeafdff1739', 'admin', '$2a$11$PSW5UiIcmDH/easBFUgRC.AqYm9K93BehlXIoihIE9uYz36.7kFEi', null);
insert into "user_role"(user_id, role_id) VALUES ('223bfb67-2e22-4ce3-a010-bfeafdff1739', 1);
