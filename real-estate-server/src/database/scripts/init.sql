create table app_location(
    id varchar(255) PRIMARY KEY,
    lat double precision,
    long double precision
);

create table subdivision(
    id varchar(255) PRIMARY KEY,
    s_name varchar(255)
);

create table subdivision_location(
    subdivision_id varchar(255) references subdivision,
    location_id varchar(255) references app_location,
    PRIMARY KEY (subdivision_id, location_id)
);

create table lot(
    l_name varchar(255),
    subdivision_id varchar(255) references subdivision,
    PRIMARY KEY (l_name, subdivision_id)
);

create table lot_location(
    l_name varchar(255),
    subdivision_id varchar(255),
    location_id varchar(255) references app_location,
    FOREIGN KEY (l_name, subdivision_id) references lot (l_name, subdivision_id),
    PRIMARY KEY(l_name, subdivision_id, location_id)
);

create table app_user(
    id varchar(255),
    uname varchar(255),
    primary key (id)
);

create table credentials(
    passwd varchar(255),
    username varchar(255),
    user_id varchar(255) references app_user,
    primary key (user_id, username)
);