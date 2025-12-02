<!-- area-id: pythios-cemetery -->
### Pythios Cemetery

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=bellsprout">Bellsprout</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="bellsprout" /> |
| <a href="../pokemon-lookup.html?q=cyndaquil">Cyndaquil</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="cyndaquil" /> |
| <a href="../pokemon-lookup.html?q=goomy">Goomy</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="goomy" /> |
| <a href="../pokemon-lookup.html?q=natu">Natu</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="natu" /> |
| <a href="../pokemon-lookup.html?q=paras">Paras</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="paras" /> |
| <a href="../pokemon-lookup.html?q=scraggy">Scraggy</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="scraggy" /> |
| <a href="../pokemon-lookup.html?q=shuppet">Shuppet</a> | 8% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="shuppet" /> |
| <a href="../pokemon-lookup.html?q=sizzlipede">Sizzlipede</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="sizzlipede" /> |
| <a href="../pokemon-lookup.html?q=trapinch">Trapinch</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="trapinch" /> |
| <a href="../pokemon-lookup.html?q=wattrel">Wattrel</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="wattrel" /> |
| <a href="../pokemon-lookup.html?q=duskull">Duskull</a> | — | 8% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="duskull" /> |
| <a href="../pokemon-lookup.html?q=ekans">Ekans</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="ekans" /> |
| <a href="../pokemon-lookup.html?q=litwick">Litwick</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="litwick" /> |
| <a href="../pokemon-lookup.html?q=snom">Snom</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="snom" /> |
| <a href="../pokemon-lookup.html?q=stunky">Stunky</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="stunky" /> |
| <a href="../pokemon-lookup.html?q=clauncher">Clauncher</a> | — | — | 10% | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="clauncher" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | 30% | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="remoraid" /> |
| <a href="../pokemon-lookup.html?q=wimpod">Wimpod</a> | — | — | 60% | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="wimpod" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=dratini">Dratini</a> | — | — | — | — | 20% | 15% | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="dratini" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=dragonair">Dragonair</a> | — | — | — | — | — | 5% | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="dragonair" /> |
| <a href="../pokemon-lookup.html?q=poliwhirl">Poliwhirl</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="pythios-cemetery" data-species="poliwhirl" /> |

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

