EXPLAIN
SELECT *
FROM public."Goods";


EXPLAIN ANALYSE
SELECT *
FROM public."Goods"
where goods_id < 20;