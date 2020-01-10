-- Your SQL goes here

Create table books (
  id int not null auto_increment,
  title varchar(200) not null,
  author varchar(200) not null,
  published boolean not null default 0,
  PRIMARY KEY(id)
);