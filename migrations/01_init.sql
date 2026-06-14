CREATE TABLE instructors (
    id SERIAL PRIMARY KEY,
    full_name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    expertise VARCHAR(100)
);

CREATE TABLE courses (
    id SERIAL PRIMARY KEY,
    title VARCHAR(150) NOT NULL,
    description TEXT,
    instructor_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),

    FOREIGN KEY (instructor_id)
        REFERENCES instructors(id)
);

CREATE TABLE students (
    id SERIAL PRIMARY KEY,
    full_name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE enrollments (
    id SERIAL PRIMARY KEY,
    student_id INT NOT NULL,
    course_id INT NOT NULL,
    enrolled_at TIMESTAMP DEFAULT NOW(),

    FOREIGN KEY (student_id)
        REFERENCES students(id),

    FOREIGN KEY (course_id)
        REFERENCES courses(id)
);