package es.uma.entity;

import lombok.Data;

import javax.persistence.*;

@Data
@Entity
@Table(name = "role_table", schema = "public", catalog = "auth")
public class RoleEntity {
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Id
    @Column(name = "id", nullable = false)
    private Integer id;
    @Basic
    @Column(name = "name", nullable = false, length = 20)
    private String name;
}
