EXPLAIN
SELECT *
FROM public."Goods";


EXPLAIN ANALYSE
SELECT public."Goods".good_name
FROM public."Goods" join public."Categories"
ON public."Goods".categories_id = public."Categories".categories_id