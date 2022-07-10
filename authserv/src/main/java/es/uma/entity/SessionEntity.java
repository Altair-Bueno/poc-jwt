package es.uma.entity;

import lombok.Data;
import org.hibernate.annotations.GenericGenerator;

import javax.persistence.*;
import java.sql.Timestamp;
import java.util.UUID;

@Data
@Entity
@Table(name = "session", schema = "public", catalog = "auth")
public class SessionEntity {
    @GeneratedValue(generator = "UUID_session")
    @GenericGenerator(
            name = "UUID_session",
            strategy = "org.hibernate.id.UUIDGenerator"
    )
    @Id
    @Column(name = "id", nullable = false)
    private UUID id;
    @Basic
    @Column(name = "deleted_at", nullable = true)
    private Timestamp deletedAt;
    @ManyToOne
    @JoinColumn(name = "user_id", referencedColumnName = "id", nullable = false)
    private UserEntity userEntity;
}
