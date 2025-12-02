<!-- area-id: wakewater-isle -->
### Wakewater Isle

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=alolan-raichu">Alolan Raichu</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="alolan-raichu" /> |
| <a href="../pokemon-lookup.html?q=bewear">Bewear</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="bewear" /> |
| <a href="../pokemon-lookup.html?q=crabominable">Crabominable</a> | 5% | 5% | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="crabominable" /> |
| <a href="../pokemon-lookup.html?q=crawdaunt">Crawdaunt</a> | 4% | 4% | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="crawdaunt" /> |
| <a href="../pokemon-lookup.html?q=dodrio">Dodrio</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="dodrio" /> |
| <a href="../pokemon-lookup.html?q=farigiraf">Farigiraf</a> | 2% | — | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="farigiraf" /> |
| <a href="../pokemon-lookup.html?q=gogoat">Gogoat</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="gogoat" /> |
| <a href="../pokemon-lookup.html?q=komala">Komala</a> | 4% | 4% | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="komala" /> |
| <a href="../pokemon-lookup.html?q=passimian">Passimian</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="passimian" /> |
| <a href="../pokemon-lookup.html?q=scyther">Scyther</a> | 5% | 5% | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="scyther" /> |
| <a href="../pokemon-lookup.html?q=toucannon">Toucannon</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="toucannon" /> |
| <a href="../pokemon-lookup.html?q=annihilape">Annihilape</a> | — | 2% | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="annihilape" /> |
| <a href="../pokemon-lookup.html?q=oranguru">Oranguru</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="oranguru" /> |
| <a href="../pokemon-lookup.html?q=pelipper">Pelipper</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="pelipper" /> |
| <a href="../pokemon-lookup.html?q=raichu">Raichu</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="raichu" /> |
| <a href="../pokemon-lookup.html?q=victreebel">Victreebel</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="wakewater-isle" data-species="victreebel" /> |

<script>
(function() {
  const STORAGE_KEY = 'lazarusEncounterChecks';
  function loadState() {
    try {
      return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}');
    } catch (_) {
      return {};
    }
  }
  function saveState(state) {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
    } catch (_) {}
  }
  const state = loadState();
  const checkboxes = document.querySelectorAll('.encounter-check');
  checkboxes.forEach(cb => {
    const key = `${cb.dataset.area}|${cb.dataset.species}`;
    cb.checked = !!state[key];
    cb.addEventListener('change', () => {
      if (cb.checked) {
        state[key] = true;
      } else {
        delete state[key];
      }
      saveState(state);
    });
  });
})();
</script>

