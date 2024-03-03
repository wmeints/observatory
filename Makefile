.PHONY: entities
entities :
	sea-orm-cli generate entity -o entity/src --with-serde both --expanded-format --lib

migrations:
	sea-orm-cli migrate up