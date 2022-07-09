package es.uma.model;

import lombok.Data;

import java.util.Date;

@Data
public class Session {
    private String id;
    private Date expires;
}
