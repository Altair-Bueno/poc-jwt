package es.uma.entity;

import org.hibernate.annotations.GenericGenerator;

import javax.persistence.*;
import java.sql.Timestamp;
import java.util.Objects;
import java.util.UUID;

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
    private UserEntity userByUserId;

    public Object getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public Timestamp getDeletedAt() {
        return deletedAt;
    }

    public void setDeletedAt(Timestamp deletedAt) {
        this.deletedAt = deletedAt;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof SessionEntity)) return false;
        SessionEntity that = (SessionEntity) o;
        return Objects.equals(id, that.id) && Objects.equals(deletedAt, that.deletedAt) && Objects.equals(userByUserId, that.userByUserId);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, deletedAt, userByUserId);
    }

    public UserEntity getUserByUserId() {
        return userByUserId;
    }

    public void setUserByUserId(UserEntity userByUserId) {
        this.userByUserId = userByUserId;
    }
}
