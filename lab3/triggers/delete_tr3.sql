begin transaction isolation level serializable;
select count(*) from public."Goods";

select count(*) from public."Goods";

commit;
select count(*) from public."Goods";