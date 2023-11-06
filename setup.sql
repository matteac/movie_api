-- POSTGRESQL

DROP TABLE IF EXISTS movie_genres;
DROP TABLE IF EXISTS genre;
DROP TABLE IF EXISTS movie;

CREATE TABLE genre (
  id SERIAL PRIMARY KEY,
  name VARCHAR(128)
);

CREATE TABLE movie (
	id UUID primary key default gen_random_uuid(),
	title VARCHAR(255) NOT NULL,
	director VARCHAR(128) NOT NULL,
	year INTEGER NOT NULL,
	duration_in_seconds INTEGER NOT NULL,
	poster TEXT NOT NULL,
	rate REAL NOT NULL,
    CHECK(year <= 2028 and rate >= 0 and rate <= 10)
);

CREATE TABLE movie_genres (
	movie_id UUID REFERENCES movie(id),
	genre_id INTEGER REFERENCES genre(id),
	PRIMARY KEY (movie_id, genre_id)
);

-- Populate 

INSERT INTO genre (name)
  VALUES
('Drama'),
('Comedy'),
('Sci-Fi'),
('Crime'),
('Adventure'),
('Romance'),
('Action');

INSERT INTO movie (title, director, year, duration_in_seconds, rate, poster)
  VALUES
('Inception', 'Christopher Nolan', 2010, 10800, 8.8, 'https://m.media-amazon.com/images/S/pv-target-images/17a24723ffa0105d2d508586e08ecf72f7e6712888e4ac1a7cad6cd52d6dcd21.jpg'),
('The Shawshank Redemption', 'Frank Darabont', 1992, 8520, 9.0, 'https://m.media-amazon.com/images/M/MV5BNDE3ODcxYzMtY2YzZC00NmNlLWJiNDMtZDViZWM2MzIxZDYwXkEyXkFqcGdeQXVyNjAwNDUxODI@._V1_.jpg'),
('The Dark Knight', 'Christopher Nolan', 2008, 9120, 9.0, 'https://m.media-amazon.com/images/M/MV5BMTMxNTMwODM0NF5BMl5BanBnXkFtZTcwODAyMTk2Mw@@._V1_.jpg');

INSERT INTO movie_genres (movie_id, genre_id) VALUES 
((SELECT id FROM movie WHERE title = 'Inception'),(SELECT id FROM genre WHERE name = 'Drama')),
((SELECT id FROM movie WHERE title = 'Inception'),(SELECT id FROM genre WHERE name = 'Action')),
((SELECT id FROM movie WHERE title = 'Inception'),(SELECT id FROM genre WHERE name = 'Sci-Fi')),
((SELECT id FROM movie WHERE title = 'The Shawshank Redemption'),(SELECT id FROM genre WHERE name = 'Drama')),
((SELECT id FROM movie WHERE title = 'The Dark Knight'),(SELECT id FROM genre WHERE name = 'Action'));

