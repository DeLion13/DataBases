begin transaction isolation level repeatable read;
	CREATE OR REPLACE FUNCTION beforeDeleteGood()
		returns trigger
		language plpgsql
	AS
	$$
	BEGIN
		DELETE FROM public."Orders_Goods" WHERE goods_id = OLD.goods_id;
		RETURN OLD;
	END;
	$$;

	CREATE TRIGGER setDelName
		BEFORE DELETE
		ON public."Goods"
		FOR EACH ROW
	EXECUTE PROCEDURE beforeDeleteGood();


	DELETE FROM public."Goods" WHERE goods_id = 28;
	
	DELETE FROM public."Goods" WHERE goods_id = 32;
commit;
	
DELETE FROM public."Goods" WHERE goods_id = 33;

DROP TRIGGER setDelName on public."Goods";