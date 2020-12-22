begin transaction isolation level read committed;
    
    CREATE OR REPLACE FUNCTION beforeUpdateGood()
    returns trigger
    language plpgsql
    AS $$
    DECLARE
        departments cursor is select * from public."Departments";
		c BOOLEAN := FALSE;
    BEGIN
        FOR department IN departments LOOP
            IF department.departments_id = NEW.departments_id THEN
                c :=TRUE;
            END IF;
        end loop;

        IF c = FALSE THEN
            NEW.departments_id = 1;
		END IF;
        return NEW;
    END;
    $$;

    CREATE TRIGGER setUpdName
            BEFORE UPDATE
            ON public."Goods"
            FOR EACH ROW
    EXECUTE PROCEDURE beforeUpdateGood();

	UPDATE public."Goods" SET good_name='Bottle', departments_id=8, categories_id=1, goods_id=10 WHERE goods_id=10;
    UPDATE public."Goods" SET good_name='Bottle', departments_id=6, categories_id=1, goods_id=10 WHERE goods_id=10;
	
	DROP TRIGGER setUpdName on public."Goods";

commit;