package es.uma.entity;

import lombok.Data;
import org.hibernate.annotations.GenericGenerator;

import javax.persistence.*;
import java.sql.Timestamp;
import java.util.List;
import java.util.UUID;

@Data
@Entity
@Table(name = "user_table", catalog = "auth")
public class UserEntity {
    @GeneratedValue(generator = "UUID_user")
    @GenericGenerator(
            name = "UUID_user",
            strategy = "org.hibernate.id.UUIDGenerator"
    )
    @Id
    @Column(name = "id", nullable = false)
    private UUID id;
    @Basic
    @Column(name = "username", nullable = false, length = 40)
    private String username;
    @Basic
    @Column(name = "password", nullable = false, length = 60)
    private String password;
    @Basic
    @Column(name = "deleted_at", nullable = true)
    private Timestamp deletedAt;
    @OneToMany(mappedBy = "userEntity")
    private List<SessionEntity> sessionEntityList;

}
