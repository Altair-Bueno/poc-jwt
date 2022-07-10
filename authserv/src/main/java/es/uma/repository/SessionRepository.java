package es.uma.repository;

import es.uma.entity.SessionEntity;
import org.springframework.data.repository.CrudRepository;

import java.util.UUID;

public interface SessionRepository extends CrudRepository<SessionEntity, UUID> {

}
