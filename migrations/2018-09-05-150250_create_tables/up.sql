CREATE TABLE project (
	id TEXT(36) PRIMARY KEY NOT NULL,
	name TEXT NOT NULL
);

insert into project values ('7aca8ada-b3a8-11e8-96f8-529269fb1459', 'Super Nice Product');
insert into project values ('7aca8eae-b3a8-11e8-96f8-529269fb1459', 'Acme Legacy System');
insert into project values ('7aca9476-b3a8-11e8-96f8-529269fb1459', 'Awesome Office Suite');

CREATE TABLE release (
	id TEXT(36) PRIMARY KEY,
	project_id TEXT(36) NOT NULL,
	version TEXT(16) NOT NULL,
	FOREIGN KEY(project_id) REFERENCES project(id)
);

insert into release values ('0a196fbc-b403-11e8-96f8-529269fb1459', '7aca8ada-b3a8-11e8-96f8-529269fb1459', '1.0.0');
insert into release values ('0a1975ac-b403-11e8-96f8-529269fb1459', '7aca8ada-b3a8-11e8-96f8-529269fb1459', '1.1.0');
insert into release values ('0a197872-b403-11e8-96f8-529269fb1459', '7aca8ada-b3a8-11e8-96f8-529269fb1459', '1.2.0');

CREATE TABLE feature (
	id TEXT(36) PRIMARY KEY,
	release_id TEXT(36) NOT NULL,
	name TEXT(512) NOT NULL,
	FOREIGN KEY(release_id) REFERENCES release(id)
);

CREATE TABLE user_story (
	id TEXT(36) PRIMARY KEY,
	feature_id TEXT(36) NOT NULL,
	name TEXT(512) NOT NULL,
	FOREIGN KEY(feature_id) REFERENCES feature(id)
);

CREATE TABLE defect (
	id TEXT(36) PRIMARY KEY,
	project_id TEXT(36),
	description NOT NULL,
	FOREIGN KEY(project_id) REFERENCES project(id)
);

CREATE TABLE defect_user_story (
	defect_id TEXT(36) ,
	user_story_id TEXT(36),
	PRIMARY KEY(defect_id, user_story_id),
	FOREIGN KEY(defect_id) REFERENCES defect(id),
	FOREIGN KEY(user_story_id) REFERENCES user_story(id)
);

CREATE TABLE defect_feature (
	defect_id TEXT(36),
	feature_id TEXT(36),
	PRIMARY KEY(defect_id, feature_id),
	FOREIGN KEY(defect_id) REFERENCES defect(id),
	FOREIGN KEY(feature_id) REFERENCES feature(id)
);