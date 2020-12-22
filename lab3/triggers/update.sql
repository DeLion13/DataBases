CREATE OR REPLACE FUNCTION afterInsertWorker()
returns trigger
language plpgsql
AS $$
BEGIN
    IF NEW.age > 30 AND NEW.position = 'manager'  THEN
        UPDATE department SET manager= NEW.full_name || ' (old)' WHERE id= NEW.department_id;
    ELSE
        UPDATE department SET manager= NEW.full_name || ' (young)' WHERE id = NEW.department_id;
    END IF;
    return NEW;
END;
$$;
