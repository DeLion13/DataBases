EXPLAIN
SELECT *
FROM public."Goods";


EXPLAIN ANALYSE
SELECT *
FROM public."Goods"
where good_name LIKE '%O%';