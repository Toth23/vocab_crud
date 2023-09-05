CREATE TABLE words (
	id	INTEGER NOT NULL,
	word	TEXT NOT NULL,
	translation	TEXT,
	date_added	TEXT NOT NULL,
	source	TEXT,
	PRIMARY KEY(id AUTOINCREMENT)
)
