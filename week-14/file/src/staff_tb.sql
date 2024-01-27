--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text,
    dno integer NOT NULL,
    staff_sal character varying(10),
    age integer,
    mobile character varying(20) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
101	Alade Joy	2	250000.0	33	08023089832
100	Mustapha Ali	3	175000.0	32	08063285831
107	Alokwe Martin	7	380000.0	48	07090082812
97	Dankade Aminat	5	550000.0	40	0902368832
108	Josiah Joshua	1	120000.0	30	08053189131
102	Makinde Mary	2	450000.0	55	09023487830
120	Adeleke Jane	4	200000.0	38	07061045862
122	Osahon Mark	6	320000.0	44	08022289842
117	Suleman Ajayi	3	800000.0	50	07030089811
104	Kuti Lawal	1	750000.0	35	09145689842
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

