<!-- area-id: kalami-city -->
### Kalami City

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=applin">Applin</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="applin" /> |
| <a href="../pokemon-lookup.html?q=buizel">Buizel</a> | 20% | 20% | — | — | 40% | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="buizel" /> |
| <a href="../pokemon-lookup.html?q=lillipup">Lillipup</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="lillipup" /> |
| <a href="../pokemon-lookup.html?q=meowth">Meowth</a> | 5% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="meowth" /> |
| <a href="../pokemon-lookup.html?q=passimian">Passimian</a> | 2% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="passimian" /> |
| <a href="../pokemon-lookup.html?q=shroodle">Shroodle</a> | 9% | 9% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="shroodle" /> |
| <a href="../pokemon-lookup.html?q=toedscool">Toedscool</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="toedscool" /> |
| <a href="../pokemon-lookup.html?q=totodile">Totodile</a> | 4% | 4% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="totodile" /> |
| <a href="../pokemon-lookup.html?q=wimpod">Wimpod</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="wimpod" /> |
| <a href="../pokemon-lookup.html?q=wingull">Wingull</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="wingull" /> |
| <a href="../pokemon-lookup.html?q=alolan-meowth">Alolan Meowth</a> | — | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="alolan-meowth" /> |
| <a href="../pokemon-lookup.html?q=murkrow">Murkrow</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="murkrow" /> |
| <a href="../pokemon-lookup.html?q=oranguru">Oranguru</a> | — | 2% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="oranguru" /> |
| <a href="../pokemon-lookup.html?q=barboach">Barboach</a> | — | — | 10% | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="barboach" /> |
| <a href="../pokemon-lookup.html?q=clauncher">Clauncher</a> | — | — | 60% | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="clauncher" /> |
| <a href="../pokemon-lookup.html?q=skrelp">Skrelp</a> | — | — | 30% | 30% | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="skrelp" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=quagsire">Quagsire</a> | — | — | — | — | — | 20% | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="quagsire" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="kalami-city" data-species="remoraid" /> |

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

