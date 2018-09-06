CREATE TABLE project (
	id TEXT(32) PRIMARY KEY NOT NULL,
	name TEXT NOT NULL
);

insert into project values ('066112f771de41b4aba5648b95476992', 'Super Nice Product');
insert into project values ('88b09cf3900a4d2bac72fabdbe24d300', 'Acme Legacy System');
insert into project values ('d1d91937138844009a9c8881631bde3a', 'Awesome Office Suite');

CREATE TABLE release (
	id TEXT(32) PRIMARY KEY,
	project_id TEXT(32) NOT NULL,
	version TEXT(16) NOT NULL,
	FOREIGN KEY(project_id) REFERENCES project(id)
);

CREATE TABLE feature (
	id TEXT(32) PRIMARY KEY,
	release_id TEXT(32) NOT NULL,
	name TEXT(512) NOT NULL,
	FOREIGN KEY(release_id) REFERENCES release(id)
);

CREATE TABLE user_story (
	id TEXT(32) PRIMARY KEY,
	feature_id TEXT(32) NOT NULL,
	name TEXT(512) NOT NULL,
	FOREIGN KEY(feature_id) REFERENCES feature(id)
);

CREATE TABLE defect (
	id TEXT(32) PRIMARY KEY,
	project_id TEXT(32),
	description NOT NULL,
	FOREIGN KEY(project_id) REFERENCES project(id)
);

CREATE TABLE defect_user_story (
	defect_id TEXT(32) ,
	user_story_id TEXT(32),
	PRIMARY KEY(defect_id, user_story_id),
	FOREIGN KEY(defect_id) REFERENCES defect(id),
	FOREIGN KEY(user_story_id) REFERENCES user_story(id)
);

CREATE TABLE defect_feature (
	defect_id TEXT(32),
	feature_id TEXT(32),
	PRIMARY KEY(defect_id, feature_id),
	FOREIGN KEY(defect_id) REFERENCES defect(id),
	FOREIGN KEY(feature_id) REFERENCES feature(id)
);