#[derive(Debug)]

struct Noeud {
    valeur: i32,
    gauche: Option<Box<Noeud>>,
    droit: Option<Box<Noeud>>,
}

impl Noeud {
    fn nouveau(valeur: i32) -> Noeud {
        Noeud { valeur: valeur, gauche: None, droit: None }
    }
    
    fn inserer(&mut self, valeur: i32) {
        if valeur < self.valeur {
            match self.gauche {
                Some(ref mut noeud) => noeud.inserer(valeur),
                None => self.gauche = Some(Box::new(Noeud::nouveau(valeur))),
            }
        } else {
            match self.droit {
                Some(ref mut noeud) => noeud.inserer(valeur),
                None => self.droit = Some(Box::new(Noeud::nouveau(valeur))),
            }
        }
    }
    
    fn rechercher(&self, valeur: i32) -> bool {
        if self.valeur == valeur {
            return true;
        }
        
        if valeur < self.valeur {
            match self.gauche {
                Some(ref noeud) => noeud.rechercher(valeur),
                None => false,
            }
        } else {
            match self.droit {
                Some(ref noeud) => noeud.rechercher(valeur),
                None => false,
            }
        }
    }

    fn supprimer(&mut self, valeur: i32) -> bool {
        if valeur < self.valeur {
            match self.gauche {
                Some(ref mut noeud) => noeud.supprimer(valeur),
                None => false,
            }
        } else if valeur > self.valeur {
            match self.droit {
                Some(ref mut noeud) => noeud.supprimer(valeur),
                None => false,
            }
        } else {
            if self.gauche.is_some() && self.droit.is_some() {
                let mut noeud_min_droit = self.droit.as_mut().unwrap();
                while let Some(ref mut sous_noeud) = noeud_min_droit.gauche {
                    noeud_min_droit = sous_noeud;
                }
                self.valeur = noeud_min_droit.valeur;
                noeud_min_droit.supprimer(noeud_min_droit.valeur)
            } else {
                let enfant = if self.gauche.is_some() {
                    self.gauche.take()
                } else {
                    self.droit.take()
                };
                if let Some(enfant) = enfant {
                    *self = *enfant;
                } else {
                    *self = Noeud::nouveau(0); // 0 is a dummy value here
                }
                true
            }
        }
    }

    fn hauteur(&self) -> i32 {
        match (self.gauche.as_ref(), self.droit.as_ref()) {
            (Some(gauche), Some(droit)) => 1 + std::cmp::max(gauche.hauteur(), droit.hauteur()),
            (Some(gauche), None) => 1 + gauche.hauteur(),
            (None, Some(droit)) => 1 + droit.hauteur(),
            (None, None) => 1,
        }
    }

    fn taille(&self) -> i32 {
        match (self.gauche.as_ref(), self.droit.as_ref()) {
            (Some(gauche), Some(droit)) => 1 + gauche.taille() + droit.taille(),
            (Some(gauche), None) => 1 + gauche.taille(),
            (None, Some(droit)) => 1 + droit.taille(),
            (None, None) => 1,
        }
    }

    fn hauteur_minimale(&self) -> i32 {
        match (self.gauche.as_ref(), self.droit.as_ref()) {
            (Some(gauche), Some(droit)) => {
                let min_hauteur = std::cmp::min(gauche.hauteur_minimale(), droit.hauteur_minimale());
                std::cmp::min(min_hauteur, 1)
            }
            (Some(gauche), None) => std::cmp::min(gauche.hauteur_minimale(), 1),
            (None, Some(droit)) => std::cmp::min(droit.hauteur_minimale(), 1),
            (None, None) => 1,
        }
    }

    fn somme(&self) -> i32 {
        self.valeur
            + self.gauche.as_ref().map(|n| n.somme()).unwrap_or(0)
            + self.droit.as_ref().map(|n| n.somme()).unwrap_or(0)
    }
    



    fn minimum(&self) -> i32 {
        match self.gauche {
            Some(ref noeud) => noeud.minimum(),
            None => self.valeur,
        }
    }
    
    fn maximum(&self) -> i32 {
        match self.droit {
            Some(ref noeud) => noeud.maximum(),
            None => self.valeur,
        }
    }

        
    // fn profondeur_noeud(&self, valeur:i32) -> Option<i32> {
    //     if !self.rechercher(valeur) {
    //         return None;
    //     } else {
    //         if self.valeur == valeur {
    //             return Some(0);
    //         } else {
    //             let gauche = self.gauche.profondeur_noeud(valeur);
    //             let droite = self.droit.profondeur_noeud(valeur);
    //             match (gauche, droite) {
    //                 (Some(g), Some(d)) => Some(1 + std::cmp::min(g, d)),
    //                 (Some(g), None) => Some(1 + g),
    //                 (None, Some(d)) => Some(1 + d),
    //                 (None, None) => None,
    //             }
    //         }
    //     }
    // }    
        
    fn predecesseur(&self, valeur: i32) -> Option<i32> {
        if valeur <= self.minimum() {
            return None;
        }
        let mut predecesseur = None;
        let mut courant = self;
        while courant.valeur != valeur {
            if courant.valeur > valeur {
                courant = courant.gauche.as_ref().unwrap();
            } else {
                predecesseur = Some(courant.valeur);
                courant = courant.droit.as_ref().unwrap();
            }
        }
        if courant.gauche.is_some() {
            predecesseur = Some(courant.gauche.as_ref().unwrap().maximum());
        }
        predecesseur
    }
    
    fn successeur(&self, valeur: i32) -> Option<i32> {
        if valeur >= self.maximum() {
            return None;
        }
        let mut successeur = None;
        let mut courant = self;
        while courant.valeur != valeur {
            if courant.valeur < valeur {
                courant = courant.droit.as_ref().unwrap();
            } else {
                successeur = Some(courant.valeur);
                courant = courant.gauche.as_ref().unwrap();
            }
        }
        if courant.droit.is_some() {
            successeur = Some(courant.droit.as_ref().unwrap().minimum());
        }
        successeur
    }

    fn parcours_prefixe(&self) -> Vec<i32> {
        let mut liste = Vec::new();
        liste.push(self.valeur);
        if let Some(gauche) = &self.gauche {
            liste.append(&mut gauche.parcours_prefixe());
        }
        if let Some(droit) = &self.droit {
            liste.append(&mut droit.parcours_prefixe());
        }
        liste
    }

    fn parcours_infixe(&self) -> Vec<i32> {
        let mut resultat = Vec::new();
        if let Some(gauche) = &self.gauche {
            resultat.append(&mut gauche.parcours_infixe());
        }
        resultat.push(self.valeur);
        if let Some(droit) = &self.droit {
            resultat.append(&mut droit.parcours_infixe());
        }
        resultat
    }

    fn parcours_suffixe(&self) -> Vec<i32> {
        let mut resultat = Vec::new();
        if let Some(gauche) = &self.gauche {
            resultat.append(&mut gauche.parcours_suffixe());
        }
        if let Some(droit) = &self.droit {
            resultat.append(&mut droit.parcours_suffixe());
        }
        resultat.push(self.valeur);
        resultat
    }

}

fn main(){
    let mut arbre = Noeud::nouveau(1);
    println!("{:?}", arbre);
    arbre.inserer(2);
    arbre.inserer(3);
    arbre.inserer(4);
    arbre.inserer(5);
    arbre.inserer(6);   
    arbre.inserer(7);   

    println!("{:?}", arbre);
    //arbre.maximum();
    //arbre.minimum();
    //arbre.taille();
    //arbre.hauteur();
    //arbre.rechercher(5);
    //arbre.supprimer(5);
    //arbre.predecesseur(1);
    //arbre.successeur(1);
    println!("{:?}", arbre);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inserer() {
        let mut arbre = Noeud::nouveau(5);
        arbre.inserer(3);
        arbre.inserer(7);
        arbre.inserer(1);
        arbre.inserer(9);

        assert_eq!(arbre.valeur, 5);

        let gauche = arbre.gauche.unwrap();
        assert_eq!(gauche.valeur, 3);

        let gauche_gauche = gauche.gauche.unwrap();
        assert_eq!(gauche_gauche.valeur, 1);

        let droit = arbre.droit.unwrap();
        assert_eq!(droit.valeur, 7);

        let droit_droit = droit.droit.unwrap();
        assert_eq!(droit_droit.valeur, 9);
    }

    #[test]
    fn test_rechercher() {
        let mut arbre = Noeud::nouveau(5);
        arbre.inserer(3);
        arbre.inserer(7);
        arbre.inserer(1);
        arbre.inserer(9);

        assert!(arbre.rechercher(5));
        assert!(!arbre.rechercher(10));
    }

    #[test]
    fn test_supprimer() {
        let mut arbre = Noeud::nouveau(5);
        arbre.inserer(3);
        arbre.inserer(7);
        arbre.inserer(1);
        arbre.inserer(9);
        
        assert_eq!(arbre.rechercher(3), true);
        assert_eq!(arbre.supprimer(3), true);
        assert_eq!(arbre.rechercher(3), false);
    }

    #[test]
    fn test_hauteur_et_taille() {
        let mut arbre = Noeud::nouveau(5);
        arbre.inserer(3);
        arbre.inserer(7);
        arbre.inserer(1);
        arbre.inserer(9);
        arbre.inserer(6);
        arbre.inserer(8);
        
        assert_eq!(arbre.hauteur(), 4);
        assert_eq!(arbre.taille(), 7);
    }

    #[test]
    fn test_minimum() {
        let mut arbre = Noeud::nouveau(5);
        arbre.inserer(3);
        arbre.inserer(7);
        arbre.inserer(1);
        arbre.inserer(9);
        assert_eq!(arbre.minimum(), 1);
    }

    #[test]
    fn test_maximum() {
        let mut arbre = Noeud::nouveau(5);
        arbre.inserer(3);
        arbre.inserer(7);
        arbre.inserer(1);
        arbre.inserer(9);
        assert_eq!(arbre.maximum(), 9);
    }

    #[test]
    fn test_predecesseur() {
        let mut arbre = Noeud::nouveau(5);
        arbre.inserer(3);
        arbre.inserer(7);
        arbre.inserer(1);
        arbre.inserer(9);

        assert_eq!(arbre.predecesseur(7).unwrap(), 5);
        assert_eq!(arbre.predecesseur(1), None);
    }

    #[test]
    fn test_successeur() {
        let mut arbre = Noeud::nouveau(5);
        arbre.inserer(3);
        arbre.inserer(7);
        arbre.inserer(1);
        arbre.inserer(9);

        assert_eq!(arbre.successeur(7).unwrap(), 9);
        assert_eq!(arbre.successeur(9), None);
    }

    #[test]
    fn test_parcours_prefixe() {
        let mut arbre = Noeud::nouveau(5);
        arbre.inserer(3);
        arbre.inserer(7);
        arbre.inserer(1);
        arbre.inserer(9);
    
        assert_eq!(arbre.parcours_prefixe(), vec![5, 3, 1, 7, 9]);
    }

    #[test]
    fn test_parcours_infixe() {
        let mut arbre = Noeud::nouveau(5);
        arbre.inserer(3);
        arbre.inserer(7);
        arbre.inserer(1);
        arbre.inserer(9);
    
        assert_eq!(arbre.parcours_infixe(), vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_parcours_suffixe() {
        let mut arbre = Noeud::nouveau(5);
        arbre.inserer(3);
        arbre.inserer(7);
        arbre.inserer(1);
        arbre.inserer(9);
    
        assert_eq!(arbre.parcours_suffixe(), vec![1, 3, 9, 7, 5]);
    }

    // #[test]
    // fn test_profondeur_noeud(){
    //     let mut arbre = Noeud::nouveau(5);
    //     arbre.inserer(3);
    //     arbre.inserer(7);
    //     arbre.inserer(1);
    //     arbre.inserer(9);

    //     assert_eq!(arbre.profondeur_noeud(1), 2);
    // }

    #[test]
    fn test_somme() {
        let mut arbre = Noeud::nouveau(5);
        arbre.inserer(3);
        arbre.inserer(7);
        arbre.inserer(1);
        arbre.inserer(9);
    
        assert_eq!(arbre.somme(), 25);
    }
    
}
